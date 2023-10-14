defmodule BankAccount do
  use Agent

  @initial_balance 0

  def open() do
    {:ok, pid} = Agent.start_link(fn -> @initial_balance end)
    pid
  end

  def close(account) do
    Agent.stop(account)
  end

  def balance(account) do
    case fetch(account) do
      {:ok, balance} -> balance
      error -> error
    end
  end

  def deposit(account, amount) do
    with {:ok, _balance} <- fetch(account),
         true <- amount_positive?(amount) do
      Agent.cast(account, &(&1 + amount))
    end
  end

  def withdraw(account, amount) do
    with {:ok, balance} <- fetch(account),
         true <- enough_balance?(balance, amount),
         true <- amount_positive?(amount) do
      Agent.cast(account, &(&1 - amount))
    end
  end

  defp fetch(account) do
    if Process.alive?(account) do
      {:ok, Agent.get(account, & &1)}
    else
      {:error, :account_closed}
    end
  end

  defp enough_balance?(balance, amount) when balance >= amount, do: true
  defp enough_balance?(_balance, _amount), do: {:error, :not_enough_balance}

  defp amount_positive?(amount) when amount > 0, do: true
  defp amount_positive?(_amount), do: {:error, :amount_must_be_positive}
end
