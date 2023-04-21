use crossfud::AsAny;

struct Test;

impl AsAny for Test {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}