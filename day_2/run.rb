require_relative './input.rb'

class Submarine
  attr_accessor :position, :depth, :aim

  def initialize
    @position = 0
    @depth = 0
    @aim = 0
  end
end

class BaseCommand
  attr_reader :direction, :distance

  def initialize(direction=0, distance=0)
    raise ArgumentError, 'direction must be one of forward, down, up' unless %w[forward down up].include?(direction)
    @direction = direction.to_sym
    @distance = distance.to_i
  end
end

class NormalCommand < BaseCommand
  def execute(submarine)
    case direction
    when :forward
      submarine.position += distance
    when :down
      submarine.depth += @distance
    when :up
      submarine.depth -= @distance
    end
  end
end

class CommandWithAim < BaseCommand
 def execute(submarine)
    case direction
    when :forward
      submarine.position += distance
      submarine.depth += submarine.aim * distance
    when :down
      submarine.aim += @distance
    when :up
      submarine.aim -= @distance
    end
  end
end

#day_1
begin
  submarine = Submarine.new

  INPUT.map do |command|
    direction, distance = command.split(' ')
    NormalCommand.new(direction, distance).execute(submarine)
  end

  puts submarine.position * submarine.depth
end

# day_2
begin
  submarine = Submarine.new

  INPUT.map do |command|
    direction, distance = command.split(' ')
    CommandWithAim.new(direction, distance).execute(submarine)
  end

  puts submarine.position * submarine.depth
end