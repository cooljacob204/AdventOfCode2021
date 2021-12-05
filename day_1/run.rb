require_relative './input.rb'

#part 1
puts INPUT.each_with_index.reduce(0) { |sum, (value, index)|
  if INPUT[index + 1] && value < INPUT[index + 1]
    sum + 1 
  else
    sum
  end
}

#part 2
puts INPUT.each_with_index.reduce(0) { |sum, (_value, index)|
  break sum unless INPUT[index + 3]

  next_3 = INPUT[index + 1] + INPUT[index + 2] + INPUT[index + 3]
  current_3 = INPUT[index + 0] + INPUT[index + 1] + INPUT[index + 2]

  if current_3 < next_3
    sum + 1 
  else
    sum
  end
}
