alter table loks
    add has_decoder boolean not null default false;
alter table loks
    add image_path varchar(255) default null;
