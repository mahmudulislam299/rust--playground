mod folder;


// use folder::
use folder::{print,subfolder1::subfile1, subfolder2::subfile2};


fn main() {
    print::printfolder();
    subfile1::printsubfile1();
    subfile2::printsubfile2();
}
