table! {
    printers (id) {
        id -> Integer,
        name -> Text,
        status -> Text,
        active -> Bool,
        selectable -> Bool,
        nametag_id -> Nullable<Integer>,
        color -> Text,
        ip -> Text,
        api_key -> Text,
        slic3r_conf -> Text,
    }
}

table! {
    nametags (id) {
        id -> Integer,
        name -> Text,
        comments -> Nullable<Text>,
    }
}
