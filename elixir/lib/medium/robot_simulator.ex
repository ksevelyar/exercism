defmodule RobotSimulator do
  @directions [:north, :east, :south, :west]

  defstruct [:direction, :position]

  def create(direction \\ :north, position \\ {0, 0})

  def create(direction, _) when direction not in @directions, do: {:error, "invalid direction"}

  def create(direction, {x, y}) when is_integer(x) and is_integer(y) do
    %RobotSimulator{direction: direction, position: {x, y}}
  end

  def create(_direction, _), do: {:error, "invalid position"}

  def simulate(robot, instructions) do
    instructions = String.codepoints(instructions)

    Enum.reduce_while(instructions, robot, fn instruction, robot ->
      case instruction do
        "R" -> {:cont, turn_right(robot)}
        "L" -> {:cont, turn_left(robot)}
        "A" -> {:cont, advance(robot)}
        _ -> {:halt, {:error, "invalid instruction"}}
      end
    end)
  end

  defp turn_right(%{direction: :north} = robot), do: %{robot | direction: :east}
  defp turn_right(%{direction: :east} = robot), do: %{robot | direction: :south}
  defp turn_right(%{direction: :south} = robot), do: %{robot | direction: :west}
  defp turn_right(%{direction: :west} = robot), do: %{robot | direction: :north}

  defp turn_left(%{direction: :north} = robot), do: %{robot | direction: :west}
  defp turn_left(%{direction: :west} = robot), do: %{robot | direction: :south}
  defp turn_left(%{direction: :south} = robot), do: %{robot | direction: :east}
  defp turn_left(%{direction: :east} = robot), do: %{robot | direction: :north}

  defp advance(%{direction: direction, position: {x, y}} = robot) do
    case direction do
      :north -> %{robot | position: {x, y + 1}}
      :east -> %{robot | position: {x + 1, y}}
      :south -> %{robot | position: {x, y - 1}}
      :west -> %{robot | position: {x - 1, y}}
    end
  end

  def direction(%RobotSimulator{direction: direction}), do: direction
  def position(%RobotSimulator{position: position}), do: position
end
