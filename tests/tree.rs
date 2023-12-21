use keetree::Node;

fn parse(route: &str) -> impl Iterator<Item = &str> {
    route.trim_start_matches('/').split('/')
}

macro_rules! match_tests {
    ($($name:ident {
        routes = $routes:expr,
        $( $path:literal :: $route:literal =>
            $( $(@$none:tt)? None )?
            $( $(@$some:tt)? { $( $key:literal => $val:literal ),* $(,)? } )?
        ),* $(,)?
    }),* $(,)?) => { $(
        #[test]
        fn $name() {
            let mut router = Node::default();

            for route in $routes {
                router.insert(parse(route), route.to_owned())
            }

            $(match router.at(parse($path)) {
                Some(result) => {
                    $($( @$some )?
                        if result != $route {
                            panic!(
                                "Wrong value for route '{}'. Expected '{}', found '{}')",
                                $path, $route, result
                            );
                        }
                    )?
                    $($( @$none )?
                        panic!("Expected no value for route '{}'", $path)
                    )?
                }
                None => {
                    $($( @$some )?
                        panic!("Expected value for route '{}'", $path)
                    )?
                }
            })*
        }
   )* };
}

match_tests! {
    basic {
        routes = [
            "/hi",
            "/contact",
            "/co",
            "/c",
            "/a",
            "/ab",
            "/doc/",
            "/doc/rust_faq.html",
            "/doc/rust1.26.html",
            "/ʯ",
            "/β",
            "/sd!here",
            "/sd$here",
            "/sd&here",
            "/sd'here",
            "/sd(here",
            "/sd)here",
            "/sd+here",
            "/sd,here",
            "/sd;here",
            "/sd=here",
        ],
        "/a"       :: "/a"       => {},
        ""         :: "/"        => None,
        "/hi"      :: "/hi"      => {},
        "/contact" :: "/contact" => {},
        "/co"      :: "/co"      => {},
        ""         :: "/con"     => None,
        ""         :: "/cona"    => None,
        ""         :: "/no"      => None,
        "/ab"      :: "/ab"      => {},
        "/ʯ"       :: "/ʯ"       => {},
        "/β"       :: "/β"       => {},
        "/sd!here" :: "/sd!here" => {},
        "/sd$here" :: "/sd$here" => {},
        "/sd&here" :: "/sd&here" => {},
        "/sd'here" :: "/sd'here" => {},
        "/sd(here" :: "/sd(here" => {},
        "/sd)here" :: "/sd)here" => {},
        "/sd+here" :: "/sd+here" => {},
        "/sd,here" :: "/sd,here" => {},
        "/sd;here" :: "/sd;here" => {},
        "/sd=here" :: "/sd=here" => {},
    },
}
