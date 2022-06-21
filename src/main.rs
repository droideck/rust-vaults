use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct Item {
    id: usize,
    title: String,
}

#[derive(Properties, PartialEq)]
struct ItemListProps {
    items: Vec<Item>,
}

#[function_component(ItemList)]
fn items_list(ItemListProps { items }: &ItemListProps) -> Html {
    items.iter().map(|item| html! {
        <p>{format!("Name - {} [ID: {}]", item.title, item.id)}</p>
    }).collect()
}

#[function_component(App)]
fn app() -> Html {
    let items = vec![
        Item {
            id: 1,
            title: "Armor".to_string(),
        },
        Item {
            id: 2,
            title: "Book".to_string(),
        },
        Item {
            id: 3,
            title: "Dice set".to_string(),
        },
    ];

    html! {
    <>
        <h1>{ "Inventory" }</h1>
        <div>
            <ItemList items={items} />
        </div>
    </>
}
}

fn main() {
    yew::start_app::<App>();
} 
