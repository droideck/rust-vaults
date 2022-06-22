use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct Item {
    id: usize,
    title: String,
}

#[derive(Properties, PartialEq)]
struct ItemListProps {
    items: Vec<Item>,
    on_click: Callback<Item>
}

#[derive(Clone, Properties, PartialEq)]
struct ItemDetailsProps {
    item: Item,
}

#[function_component(ItemDetails)]
fn item_details(ItemDetailsProps { item }: &ItemDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ item.title.clone() }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}

#[function_component(ItemList)]
fn items_list(ItemListProps { items, on_click }: &ItemListProps) -> Html {
    let on_click = on_click.clone();
    items.iter().map(|item| {
        let on_item_select = {
            let on_click = on_click.clone();
            let item = item.clone();
            Callback::from(move |_| {
                on_click.emit(item.clone())
            })
        };
        html! {
            <p onclick={on_item_select}>{format!("Name {}: [ID: {}]", item.title, item.id)}</p>
        }
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
    let selected_item = use_state(|| None);
    let on_item_select = {
        let selected_item = selected_item.clone();
        Callback::from(move |item: Item| {
            selected_item.set(Some(item))
        })
    };
    let details = selected_item.as_ref().map(|item| html! {
        <ItemDetails item={item.clone()} />
    });

    html! {
    <>
        <h1>{ "Inventory" }</h1>
        <div>
            <ItemList items={items} on_click={on_item_select.clone()} />
        </div>
        { for details }
    </>
}
}

fn main() {
    yew::start_app::<App>();
} 
