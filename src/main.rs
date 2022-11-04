use wants_needs_rust::classifier::*;
use wants_needs_rust::ynab_api::YnabApi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut api = YnabApi::new("u_twETDNkzhsGneWEtcW_7_P2jlhtb_huCwBFOAzw8g", "ynab.cache");
    let budget_summary = api.list_budgets(false).await?;
    dbg!(&budget_summary);
    let my_budget = &budget_summary.data.budgets.iter().filter(|b| b.name.contains("My Budget")).next().unwrap();
    let budget_id = &my_budget.id;
    

    let categories = api
        .list_categories(budget_id)
        .await?;

    let mut income = 0;
    let mut expense_data = vec![];
    for c in categories
        .data
        .category_groups
        .into_iter()
        .flat_map(|cg| cg.categories)
    {
        dbg!(&c.name);
        let resp = api.get_category_transactions(budget_id, &c.id).await?;
        let total = resp.data.transactions.iter().map(|t| t.amount).sum::<i64>();
        if total > 0 {
            income += total;
        } else if total < 0 {
            expense_data.push((c.name, total.abs()));
        }
    }

    dbg!(&income);
    dbg!(&expense_data);
    let mut cls = Classifier::new("classes.json");
    let (wants, needs, mut save, loan) = expense_data.into_iter().fold((0, 0, 0, 0), |mut acc, ed| {
        match cls.classify(&ed.0) {
            Class::Want => acc.0 += ed.1,
            Class::Need => acc.1 += ed.1,
            Class::Save => acc.2 += ed.1,
            Class::Loan => acc.3 += ed.1,
        }
        acc
    });
    save += income - wants - needs - loan - save;

    let percent_wants = wants as f64 / income as f64 * 100.;
    let percent_needs = needs as f64 / income as f64 * 100.;
    let percent_loans = loan as f64 / income as f64 * 100.;
    let percent_saves = save as f64 / income as f64 * 100.;

    println!("Spent on wants:    ${:>10.2}  {:>10.2}%", wants as f64 / 1000., percent_wants);
    println!("Spent on needs:    ${:>10.2}  {:>10.2}%", needs as f64 / 1000., percent_needs);
    println!("Spent on loans:    ${:>10.2}  {:>10.2}%", loan as f64 / 1000., percent_loans);
    println!("         Saved:    ${:>10.2}  {:>10.2}%", save as f64 / 1000., percent_saves);

    Ok(())
}
