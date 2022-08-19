defmodule RPNCalculator.Exception do
  defmodule DivisionByZeroError do
    defexception message: "division by zero occurred"
  end

  defmodule StackUnderflowError do
    defexception message: "stack underflow occurred"

    @impl true
    def exception(context) do
      case context do
        [] ->
          %StackUnderflowError{}

        _ ->
          message = %StackUnderflowError{}.message
          %StackUnderflowError{message: "#{message}, context: #{context}"}
      end
    end
  end

  def divide(stack) when length(stack) != 2, do: raise(StackUnderflowError, "when dividing")
  def divide([divisor | _]) when divisor == 0, do: raise(DivisionByZeroError)
  def divide([divisor | [dividend]]), do: dividend / divisor
end
