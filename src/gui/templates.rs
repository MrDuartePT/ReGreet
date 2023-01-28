// SPDX-FileCopyrightText: 2022 Harish Rajagopal <harish.rajagopals@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Templates for various GUI components
use gtk::prelude::*;
use relm4::{gtk, RelmWidgetExt, WidgetTemplate};

/// Button that ends the greeter (eg. Reboot)
#[relm4::widget_template(pub)]
impl WidgetTemplate for EndButton {
    view! {
        gtk::Button {
            set_focusable: true,
            add_css_class: "destructive-action",
        }
    }
}

/// Label for an entry/combo box
#[relm4::widget_template(pub)]
impl WidgetTemplate for EntryLabel {
    view! {
        gtk::Label {
            set_width_request: 100,
            set_xalign: 1.0,
        }
    }
}

/// Main UI of the greeter
#[relm4::widget_template(pub)]
impl WidgetTemplate for Ui {
    view! {
        gtk::Overlay {
            /// Background image
            #[name = "background"]
            gtk::Picture,

            /// Main login box
            add_overlay = &gtk::Frame {
                set_halign: gtk::Align::Center,
                set_valign: gtk::Align::Center,
                inline_css: "background-color: @theme_bg_color",

                gtk::Grid {
                    set_column_spacing: 15,
                    set_margin_bottom: 15,
                    set_margin_end: 15,
                    set_margin_start: 15,
                    set_margin_top: 15,
                    set_row_spacing: 15,
                    set_width_request: 500,

                    /// Widget to display messages to the user
                    #[name = "message_label"]
                    attach[1, 0, 1, 1] = &gtk::Label {
                        set_hexpand: true,
                        set_margin_bottom: 15,
                        set_xalign: 0.0,

                        // Format all messages in boldface.
                        #[wrap(Some)]
                        set_attributes = &gtk::pango::AttrList {
                            insert: {
                                let mut font_desc = gtk::pango::FontDescription::new();
                                font_desc.set_weight(gtk::pango::Weight::Bold);
                                gtk::pango::AttrFontDesc::new(&font_desc)
                            },
                        },
                    },

                    #[template]
                    attach[0, 1, 1, 1] = &EntryLabel { set_label: "User:" },

                    /// Label for the sessions widget
                    #[name = "session_label"]
                    #[template]
                    attach[0, 2, 1, 1] = &EntryLabel { set_label: "Session:" },

                    /// Widget containing the usernames
                    #[name = "usernames_box"]
                    attach[1, 1, 1, 1] = &gtk::ComboBoxText::with_entry(),

                    /// Widget containing the sessions
                    #[name = "sessions_box"]
                    attach[1, 2, 1, 1] = &gtk::ComboBoxText::with_entry(),

                    /// Label for the password widget
                    #[name = "password_label"]
                    #[template]
                    attach[0, 2, 1, 1] = &EntryLabel { set_label: "Password:" },

                    /// Widget where the user enters the password
                    #[name = "password_entry"]
                    attach[1, 2, 1, 1] = &gtk::PasswordEntry { set_show_peek_icon: true },

                    /// Collection of action buttons (eg. Login)
                    attach[1, 3, 1, 1] = &gtk::Box {
                        set_halign: gtk::Align::End,
                        set_spacing: 15,

                        /// Button to cancel password entry
                        #[name = "cancel_button"]
                        gtk::Button {
                            set_focusable: true,
                            set_label: "Cancel",
                        },

                        /// Button to enter the password and login
                        #[name = "login_button"]
                        gtk::Button {
                            set_focusable: true,
                            set_label: "Login",
                            set_receives_default: true,
                            add_css_class: "suggested-action",
                        },
                    },
                }
            },

            /// Clock widget
            add_overlay = &gtk::Frame {
                set_halign: gtk::Align::Center,
                set_valign: gtk::Align::Start,
                // Make it fit cleanly onto the top edge of the screen.
                inline_css: "
                    border-top-right-radius: 0px;
                    border-top-left-radius: 0px;
                    border-top-width: 0px;
                    background-color: @theme_bg_color;
                ",

                /// Label displaying the current date & time
                #[name = "datetime_label"]
                gtk::Label { set_width_request: 150 },
            },

            /// Collection of buttons that close the greeter (eg. Reboot)
            add_overlay = &gtk::Box {
                set_halign: gtk::Align::Center,
                set_valign: gtk::Align::End,
                set_homogeneous: true,
                set_margin_bottom: 15,
                set_spacing: 15,

                /// Button to reboot
                #[name = "reboot_button"]
                #[template]
                EndButton { set_label: "Reboot" },

                /// Button to power-off
                #[name = "poweroff_button"]
                #[template]
                EndButton { set_label: "Power Off" },
            },
        }
    }
}