use bloc::Bloc;
use content_from_file::*;
use equations::*;
use graphics::*;
use into_tab::*;
use latex_file::LatexFile;
use math_mode::*;
/// File to define the core of a LaTex file
///
use sections::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use tabular::*;
use tag::*;
use writable::*;

// pub type LatexFile = File;

#[derive(Clone)]
pub enum Core {
    Sec(Section),
    RawText(String),
    Equa(Equation),
    Bloc(Bloc),
    Tag(SingleTag),
    Tab(Tabular),
    Math(MathContent),
    Graph(Graphic),
    Code(Code),
    TextFromFile(String),
}

impl Writable for Core {
    fn write_latex(&self, mut file: &mut LatexFile) {
        match *self {
            Core::Sec(ref section) => section.write_latex(&mut file),
            Core::RawText(ref text) => text.write_latex(&mut file),
            Core::Equa(ref eq) => eq.write_latex(&mut file),
            Core::Bloc(ref bloc) => bloc.write_latex(&mut file),
            Core::Tag(ref tag) => tag.write_latex(&mut file),
            Core::Tab(ref tab) => tab.write_latex(&mut file),
            Core::Math(ref m) => m.write_latex(&mut file),
            Core::Graph(ref g) => g.write_latex(&mut file),
            Core::Code(ref c) => c.write_latex(&mut file),
            Core::TextFromFile(ref filename) => {
                let mut out_file = File::open(filename).unwrap();
                let mut contents = String::new();
                out_file.read_to_string(&mut contents).unwrap();
                contents.write_latex(&mut file)
            }
        }
    }

    fn write_to_buffer(&self, mut buf: &mut BufWriter<&mut LatexFile>) {
        match *self {
            Core::Sec(ref section) => section.write_to_buffer(&mut buf),
            Core::RawText(ref text) => text.write_to_buffer(&mut buf),
            Core::Equa(ref eq) => eq.write_to_buffer(&mut buf),
            Core::Bloc(ref bloc) => bloc.write_to_buffer(&mut buf),
            Core::Tag(ref tag) => tag.write_to_buffer(&mut buf),
            Core::Tab(ref tab) => tab.write_to_buffer(&mut buf),
            Core::Math(ref m) => m.write_to_buffer(&mut buf),
            Core::Graph(ref g) => g.write_to_buffer(&mut buf),
            Core::Code(ref c) => c.write_to_buffer(&mut buf),
            _ => (),
        }
    }
}

impl Core {
    /// Returns a new section
    pub fn section<T: AsRef<str>>(title: T) -> Self {
        Core::Sec(Section::new_section(title.as_ref().to_string()))
    }

    /// Returns a new subsection
    pub fn subsection<T: AsRef<str>>(title: T) -> Self {
        Core::Sec(Section::new_subsection(title.as_ref().to_string()))
    }

    /// Returns a new subsubsection
    pub fn subsubsection<T: AsRef<str>>(title: T) -> Self {
        Core::Sec(Section::new_subsubsection(title.as_ref().to_string()))
    }

    /// Returns a new paragraph
    pub fn paragraph<T: AsRef<str>>(title: T) -> Self {
        Core::Sec(Section::new_paragraph(title.as_ref().to_string()))
    }

    /// Return a new text
    pub fn text<T: AsRef<str>>(raw_text: T) -> Self {
        Core::RawText(raw_text.as_ref().to_string())
    }

    /// Returns a new equation
    pub fn equation(eq: Equation) -> Self {
        Core::Equa(eq)
    }

    /// Returns a new Bloc
    pub fn bloc<T: AsRef<str>>(title: T) -> Self {
        Core::Bloc(Bloc::new_empty(title.as_ref().to_string()))
    }

    /// Returns a new Tab
    pub fn tab<T: IntoTab>(content: &T) -> Self {
        Core::Tab(Tabular::new(content))
    }

    /// Returns a new \item tag
    pub fn item(content: Core) -> Self {
        Core::Tag(SingleTag::item(content))
    }

    /// Return a math mode element
    pub fn math<T: AsRef<str>>(content: T) -> Self {
        Core::Math(MathContent::new(content.as_ref().to_string()))
    }

    /// Return a graphic element
    pub fn graphic<T1: AsRef<str>, T2: AsRef<str>>(filename: T1, description: T2) -> Self {
        Core::Graph(Graphic::new(
            filename.as_ref().to_string(),
            description.as_ref().to_string(),
        ))
    }

    /// Change the scale of the graphic
    pub fn set_scale(&mut self, new_scale: f64) {
        if let Core::Graph(ref mut graph) = *self {
            graph.set_scale(new_scale)
        }
    }

    /// Returns a new Code element
    pub fn code<T1: AsRef<str>, T2: AsRef<str>>(filename: T1, language: T2) -> Self {
        Core::Code(Code::new(
            filename.as_ref().to_string(),
            language.as_ref().to_string(),
        ))
    }

    /// Returns a new text from file element
    pub fn text_from_file<T: AsRef<str>>(filename: T) -> Self {
        Core::TextFromFile(filename.as_ref().to_string())
    }

    /// Add an element to the content, if possible
    pub fn add(&mut self, element: Core) {
        match *self {
            Core::Sec(ref mut section) => section.add_content(element),
            Core::Bloc(ref mut bloc) => bloc.add(element),
            _ => panic!("No method 'add' for this type of data"),
        }
    }
}

#[cfg(test)]
mod tests_raw_text {
    use super::*;
    use latex_file::*;

    #[test]
    fn simple_write_text() {
        let mut f = new_latex_file("./tests_results/raw_texts/simple_write.tex");
        let t1 = Core::text("Quentin");
        t1.write_latex(&mut f);
    }
}

#[cfg(test)]
mod tests_core {
    use super::*;
    use latex_file::*;

    #[test]
    fn test_enumerate() {
        let mut f = new_latex_file("./tests_results/core/enumerate.tex");
        f.begin_document();
        let mut enumerate = Core::bloc("enumerate");
        for i in 0..5 {
            enumerate.add(Core::item(Core::text(format!("Blabla {}", i))));
        }
        enumerate.write_latex(&mut f);
        f.write_footer();
    }

    #[test]
    fn test_tabular_one_dim() {
        let mut f = new_latex_file("./tests_results/core/tabular_one_dim.tex");
        f.begin_document();
        let mut vec = Vec::new();
        for i in 0..5 {
            vec.push(Core::text(i.to_string()));
        }
        // let vec = (0..5).map(|i| Core::text(i.to_string())).collect();
        let tab = Core::tab(&vec);
        tab.write_latex(&mut f);
        f.write_footer();
    }

    #[test]
    fn test_tabular_two_dims() {
        let mut f = new_latex_file("./tests_results/core/tabular_two_dims.tex");
        f.begin_document();
        let mut vec_2d = Vec::new();
        for j in 0..6 {
            let mut vec = Vec::new();
            for i in 0..5 {
                vec.push(Core::text((i + j).to_string()));
            }
            vec_2d.push(vec);
        }
        // let vec = (0..5).map(|i| Core::text(i.to_string())).collect();
        let tab = Core::tab(&vec_2d);
        tab.write_latex(&mut f);
        f.write_footer();
    }

    #[test]
    fn test_math_mode_simple() {
        let mut f = new_latex_file("./tests_results/core/math_mode_simple.tex");
        f.begin_document();
        let m = Core::math("1 + 2 = 3");
        m.write_latex(&mut f);
        f.write_footer();
    }

    #[test]
    fn test_math_mode_symbol() {
        let mut f = new_latex_file("./tests_results/core/math_mode_symbol.tex");
        f.begin_document();
        let m = Core::math(r"1 \leq 2 = 3");
        m.write_latex(&mut f);
        f.write_footer();
    }

    #[test]
    fn test_math_mode_in_text() {
        let mut f = new_latex_file("./tests_results/core/math_mode_in_text.tex");
        f.begin_document();
        let mut p = Core::paragraph("");
        p.add(Core::text("This is the most interesting equation:"));
        let m = Core::math("1 + 2 = 3");
        p.add(m);
        p.write_latex(&mut f);
        f.write_footer();
    }

    #[test]
    fn test_include_graphics() {
        let mut f = new_latex_file("./tests_results/core/include_graphics.tex");
        f.add_package("graphicx");
        f.begin_document();
        let im = Core::graphic("rust_logo.jpg", "This is the Rust logo!");
        im.write_latex(&mut f);
        f.write_footer();
    }

    #[test]
    fn test_include_graphics_scale() {
        let mut f = new_latex_file("./tests_results/core/include_graphics_scale.tex");
        f.add_package("graphicx");
        f.begin_document();
        let mut im = Core::graphic("rust_logo.jpg", "This is the Rust logo!");
        im.set_scale(0.5);
        im.write_latex(&mut f);
        f.write_footer();
    }

    #[test]
    fn test_include_code() {
        let mut f = new_latex_file("./tests_results/core/include_code.tex");
        f.add_package("listings");
        f.begin_document();
        let code = Core::code("../../test.c", "C");
        code.write_latex(&mut f);
        f.write_footer();
    }

    #[test]
    fn test_include_text() {
        let mut f = new_latex_file("./tests_results/core/include_text.tex");
        f.begin_document();
        let text = Core::text_from_file("text.txt");
        text.write_latex(&mut f);
        f.write_footer();
    }

}
