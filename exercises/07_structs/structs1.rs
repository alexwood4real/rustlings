/* C-like struct and its constuctor */
struct ColorRegularStruct {
    // TODO: Add the fields that the test `regular_structs` expects.
    // What types should the fields have? What are the minimum and maximum values for RGB colors?
    red: u32,
    green: u32,
    blue: u32
}

fn init_color( red: u32, blue: u32, green: u32 ) -> ColorRegularStruct
    {
    ColorRegularStruct
        {
        /* can do attribute: arg, but since they have the same name it is ok */
        red,
        green,
        blue
        }
    }

/* tuple struct */
struct ColorTupleStruct( u32, u32, u32 );

/* unit struct */
#[derive(Debug)]
struct UnitStruct;

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // TODO: Instantiate a regular struct.
        let green = ColorRegularStruct {
            green: 255,
            red: 0,
            blue: 0
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);

        let blue = init_color( 0, 255, 0 );

        assert_eq!(blue.red, 0);
        assert_eq!(blue.green, 0);
        assert_eq!(blue.blue, 255);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct.
        let green = ( 0, 255, 0 );

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct.
        let unit_struct = UnitStruct;
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
