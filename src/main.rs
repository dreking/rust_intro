mod operators;
use operators::operators::operators;

mod constant_static;
use constant_static::constant_static::constant_and_static;

mod data_types;
use data_types::booleans::booleans;
use data_types::characters::characters;
use data_types::floats::floats;
use data_types::integers::integers;

mod scope_shadowing;
use scope_shadowing::scope_shadowing::scope_and_shadowing;

mod stack_heap;
use stack_heap::stack_heap::stack_and_heap;

mod control_flow;
use control_flow::combination_lock::combination_lock;
use control_flow::if_statement::if_statement;
use control_flow::loop_statement::for_loop;
use control_flow::loop_statement::loop_infinite;
use control_flow::loop_statement::while_loop;
use control_flow::match_statement::match_statement;

mod data_structure;
use data_structure::array_structure::array_structure;
use data_structure::enum_structure::enum_structure;
use data_structure::generics_structure::generics_structure;
use data_structure::option_t_structure::option_t_structure;
use data_structure::pattern_matching::pattern_matching;
use data_structure::slice_structure::slice_structure;
use data_structure::struct_structure::struct_structure;
use data_structure::tuple_structure::tuple_structure;
use data_structure::union_structure::union_structure;

mod collections;
use collections::hashmap::hashmap;
use collections::hashset::hashset;
use collections::iterations::iterations;
use collections::vectors::vectors;

mod strings;
use strings::strings::formatting;
use strings::strings::strings;

mod exercises;
use exercises::number_guessing_game::number_guessing_game;

mod functions;
use functions::closures::closures;
use functions::functions::fnarguments;
use functions::functions::increase;
use functions::functions::product;
use functions::higher_order_functions::higher_order_functions;
use functions::methods::methods;

mod traits;
use traits::static_dispatch::dynamic_dispatch;
use traits::static_dispatch::static_dispatch;
use traits::traits::traits;
use traits::traits::traits_param;
use traits::traits_drop::traits_drop;
use traits::traits_into::traits_into;
use traits::traits_op_overload::traits_op_overload;
use traits::vector_multitypes::vector_multitypes;

mod lifetime;
use lifetime::borrowing::borrowing;
use lifetime::lifetime::lifetime;
use lifetime::ownership::ownership;
use lifetime::reference_counting::reference_counting;

mod circular_reference;
use circular_reference::circular_reference::circular_reference;

mod concurrency;
use concurrency::concurrency::concurrency;

mod crates;
use crates::crates::crates;
use crates::phrases::phrases;

fn main() {
    integers();
    characters();
    floats();
    booleans();
    operators();
    constant_and_static();
    scope_and_shadowing();
    stack_and_heap();
    if_statement();
    while_loop();
    loop_infinite();
    for_loop();
    match_statement();
    combination_lock();
    struct_structure();
    enum_structure();
    union_structure();
    option_t_structure();
    array_structure();
    slice_structure();
    tuple_structure();
    pattern_matching();
    generics_structure();
    vectors();
    hashmap();
    hashset();
    iterations();
    strings();
    formatting();
    number_guessing_game();
    fnarguments(5);
    increase(&mut 2);
    product(2, 3);
    methods();
    closures();
    higher_order_functions();
    traits();
    traits_param();
    traits_into();
    traits_drop();
    traits_op_overload();
    static_dispatch();
    dynamic_dispatch();
    vector_multitypes();
    ownership();
    borrowing();
    lifetime();
    reference_counting();
    circular_reference();
    concurrency();
    crates();
    phrases();
}
