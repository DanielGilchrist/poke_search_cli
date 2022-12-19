use crate::formatter::{self, FormatAbility, FormatModel};

use futures::{stream, StreamExt};
use std::{process::exit, rc::Rc};

use rustemon::{client::RustemonClient, model::pokemon::Pokemon, pokemon::pokemon, Follow};

static STAT_NAMES: &[&str] = &[
    "HP",
    "Attack",
    "Defense",
    "Special Attack",
    "Special Defense",
    "Speed",
];

pub struct PokemonCommand {
    client: RustemonClient,
    pokemon_name: String,
}

impl PokemonCommand {
    pub async fn execute(client: RustemonClient, pokemon_name: String) {
        PokemonCommand {
            client,
            pokemon_name,
        }
        ._execute()
        .await;
    }

    async fn _execute(&self) {
        let pokemon = Rc::new(self.fetch_pokemon().await);
        let mut output = String::new();

        self.build_summary(&pokemon, &mut output);
        self.build_stat_output(&pokemon, &mut output);
        self.build_ability_output(&pokemon, &mut output).await;

        println!("{}", output);
    }

    async fn fetch_pokemon(&self) -> Pokemon {
        match pokemon::get_by_name(&self.pokemon_name, &self.client).await {
            Ok(pokemon) => pokemon,
            Err(_) => {
                println!("Pokemon \"{}\" doesn't exist", self.pokemon_name);
                exit(1);
            }
        }
    }

    fn build_summary(&self, pokemon: &Rc<Pokemon>, output: &mut String) {
        output.push_str("Summary\n");
        output.push_str(&pokemon.format());
    }

    fn build_stat_output(&self, pokemon: &Rc<Pokemon>, output: &mut String) {
        output.push_str("\nStats\n");
        let mut stat_total = 0;
        pokemon.stats.iter().enumerate().for_each(|(index, stat)| {
            // This assumes the stats returned from the API are always in the same order.
            // Because "PokemonStat" doesn't include the stats name, this is much simplier
            // than requesting for the Stat resource just for the corresponding name
            let stat_name = STAT_NAMES[index];
            let stat_amount = stat.base_stat;
            stat_total += stat_amount;
            output.push_str(&formatter::formatln(stat_name, &stat_amount.to_string()));
        });
        output.push_str(&formatter::formatln("Total", &stat_total.to_string()));
    }

    async fn build_ability_output(&self, pokemon: &Rc<Pokemon>, output: &mut String) {
        output.push_str("\nAbilities\n");
        stream::iter(&pokemon.abilities)
            .map(|a| {
                let client_ref = &self.client;
                let pokemon_ref = &pokemon;

                async move {
                    let ability = a.ability.follow(client_ref).await.unwrap();

                    FormatAbility::new(ability, Rc::clone(pokemon_ref))
                }
            })
            .buffer_unordered(2)
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .for_each(|ability| {
                output.push_str(&format!("{}\n", ability.format()));
            });
    }
}