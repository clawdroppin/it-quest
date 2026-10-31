@@lesson xp-sales_analytics Sales Analytics with Window Functions
@@text
Load a year of shop orders and answer what a business analyst would ask: monthly revenue and growth, running totals, each customer's first and latest order, top 3 products per category, customers who haven't ordered in 90 days, and a cohort table of retention by first-order month.

**You'll practise:** CTEs · window functions (ROW_NUMBER, LAG, SUM OVER) · date functions · cohort analysis

## Milestones

1. **Schema and data**: orders and products tables (generate a year of data with a script or by hand).
2. **Growth**: monthly revenue with LAG for the change from last month.
3. **Ranking**: top 3 products per category with ROW_NUMBER.
4. **Churn**: customers inactive for 90 days.
5. **Cohorts**: first-order month vs months since, counting active customers.

## You're done when

- Every query runs, and you can explain each result in one sentence a manager would understand.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"Explain PARTITION BY versus GROUP BY with my data."*
- *"How do I build a cohort table in SQLite?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project sales_analytics Sales Analytics with Window Functions
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Sales Analytics with Window Functions

Load a year of shop orders and answer what a business analyst would ask: monthly revenue and growth, running totals, each customer's first and latest order, top 3 products per category, customers who haven't ordered in 90 days, and a cohort table of retention by first-order month.

## Milestones

- [ ] 1. Schema and data: orders and products tables (generate a year of data with a script or by hand).
- [ ] 2. Growth: monthly revenue with LAG for the change from last month.
- [ ] 3. Ranking: top 3 products per category with ROW_NUMBER.
- [ ] 4. Churn: customers inactive for 90 days.
- [ ] 5. Cohorts: first-order month vs months since, counting active customers.

## Done when

- [ ] Every query runs, and you can explain each result in one sentence a manager would understand.
@@file schema.sql
CREATE TABLE product (id INTEGER PRIMARY KEY, name TEXT NOT NULL, category TEXT NOT NULL, price INTEGER NOT NULL);
CREATE TABLE orders (id INTEGER PRIMARY KEY, customer TEXT NOT NULL, product_id INTEGER NOT NULL REFERENCES product(id),
                     qty INTEGER NOT NULL, ordered_on TEXT NOT NULL);
@@file queries.sql
-- Monthly revenue and change from the previous month
WITH monthly AS (
  SELECT substr(o.ordered_on, 1, 7) AS month, SUM(o.qty * p.price) AS revenue
  FROM orders o JOIN product p ON p.id = o.product_id
  GROUP BY month
)
SELECT month, revenue, revenue - LAG(revenue) OVER (ORDER BY month) AS change
FROM monthly;

-- TODO: top 3 per category, inactive customers, cohorts
