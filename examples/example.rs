extern crate RusTex;
use RusTex::core::*;
use RusTex::latex_file::*;
use RusTex::writable::Writable;

fn main() {
    let mut f = new_latex_file("out.tex");
    f.title("Example of use of RusTex");
    f.author("GuilloteauQ");
    f.begin_document();

    let mut abstract_bloc = Core::new_bloc("abstract");
    abstract_bloc.add(Core::text("This document is an example of use of RusTex"));
    abstract_bloc.write_latex(&mut f);

    let mut sec = Core::new_section("Examples");
    let mut enume = Core::new_bloc("itemize");

    let countries = vec!["France", "UK", "Germany", "Italy"];
    sec.add(Core::text("Here is some countries in Europe"));
    for country in countries.iter() {
        enume.add(Core::item(Core::text(*country)));
    }

    sec.add(enume);
    sec.write_latex(&mut f);

    f.write_footer();
}