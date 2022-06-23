use yew::prelude::*;
use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
struct Monster {
    id: usize,
    title: String,

}

#[derive(Properties, PartialEq)]
struct MonsterListProps {
    monsters: Vec<Monster>,
    on_click: Callback<Monster>
}

#[derive(Clone, Properties, PartialEq)]
struct MonsterDetailsProps {
    monster: Monster,
}

#[function_component(MonsterDetails)]
fn monster_details(MonsterDetailsProps { monster }: &MonsterDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ monster.title.clone() }</h3>
            <img src={format!("https://2e.aonprd.com/Images/Monsters/{}.png", monster.title.clone())} />
        </div>
    }
}

#[function_component(MonsterList)]
fn monsters_list(MonsterListProps { monsters, on_click }: &MonsterListProps) -> Html {
    let on_click = on_click.clone();
    monsters.iter().map(|monster| {
        let on_monster_select = {
            let on_click = on_click.clone();
            let monster = monster.clone();
            Callback::from(move |_| {
                on_click.emit(monster.clone())
            })
        };
        html! {
            <p onclick={on_monster_select}>{format!("Name: {} [ID: {}]", monster.title, monster.id)}</p>
        }
    }).collect()
}

#[function_component(App)]
fn app() -> Html {
    let monsters = vec![
        Monster {
            id: 1,
            title: "Unseen Servant".to_string(),
        },
        Monster {
            id: 541,
            title: "Akata".to_string(),
        },
        Monster {
            id: 1206,
            title: "Kappa".to_string(),
        },
    ];
    let selected_monster = use_state(|| None);
    let on_monster_select = {
        let selected_monster = selected_monster.clone();
        Callback::from(move |monster: Monster| {
            selected_monster.set(Some(monster))
        })
    };
    let details = selected_monster.as_ref().map(|monster| html! {
        <MonsterDetails monster={monster.clone()} />
    });

    html! {
    <>
        <h1>{ "Monsters" }</h1>
        <div>
            <MonsterList monsters={monsters} on_click={on_monster_select.clone()} />
        </div>
        { for details }
    </>
}
}

fn main() {
    yew::start_app::<App>();
} 
