defmodule DancingDots.Animation do
  @type dot :: DancingDots.Dot.t()
  @type opts :: keyword
  @type error :: any
  @type frame_number :: pos_integer

  @callback init(opts) :: {:ok, opts} | {:error, opts}
  @callback handle_frame(dot, frame_number, opts) :: dot

  defmacro __using__(_) do
    quote do
      @behaviour DancingDots.Animation
      def init(opts), do: {:ok, opts}
      defoverridable init: 1
    end
  end
end

defmodule DancingDots.Flicker do
  use DancingDots.Animation

  defguardp multiple_of_four?(num) when rem(num, 4) == 0

  @impl DancingDots.Animation
  def handle_frame(%{opacity: opacity} = dot, frame_number, _opts)
      when multiple_of_four?(frame_number) do
    %{dot | opacity: opacity / 2}
  end

  @impl DancingDots.Animation
  def handle_frame(dot, _frame_number, _opts), do: dot
end

defmodule DancingDots.Zoom do
  use DancingDots.Animation

  @impl DancingDots.Animation
  def init(opts) do
    velocity = Keyword.keyword?(opts) && Keyword.get(opts, :velocity)

    case is_number(velocity) do
      true ->
        {:ok, opts}

      false ->
        {:error,
         "The :velocity option is required, and its value must be a number. Got: #{inspect(velocity)}"}
    end
  end

  @impl DancingDots.Animation
  def handle_frame(%{radius: radius} = dot, frame_number, opts) do
    velocity = Keyword.get(opts, :velocity)

    %{dot | radius: radius + (frame_number - 1) * velocity}
  end
end
