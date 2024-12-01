mod dir_2023; // Modul deklarieren

fn main() {
    dir_2023::dayonea().expect("Fehler bei der Ausführung von der Datei dayone.rs");
    dir_2023::dayoneb().expect("Fehler bei der 'Ausfürhung von der Datei dayone.rs");
    dir_2023::daytwoa().expect("Fehler!");
    dir_2024::dayonea().expect("Fehler in dir_2024");
    // dir_2024::daytwob().expect("Fehler in dir_2024")
}
