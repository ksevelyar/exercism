defmodule FreelancerRates do
  def daily_rate(hourly_rate) do
    hourly_rate * 8.0
  end

  def apply_discount(before_discount, discount) do
    before_discount - discount / 100 * before_discount
  end

  def monthly_rate(hourly_rate, discount) do
    billable_days = 22
    amount = daily_rate(hourly_rate) * billable_days

    apply_discount(amount, discount) |> Float.ceil() |> trunc
  end

  def days_in_budget(budget, hourly_rate, discount) do
    discounted_rate = apply_discount(daily_rate(hourly_rate), discount)

    (budget / discounted_rate) |> Float.floor(1)
  end
end
