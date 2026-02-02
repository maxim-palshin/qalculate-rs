#include <libqalculate/Calculator.h>

extern "C" {
struct QalculateState {
  void *calc;
};

QalculateState init() {
  QalculateState state;
  state.calc = new Calculator();
  return state;
}

double calculate(const char *expression, QalculateState *state) {
  auto calc = reinterpret_cast<Calculator *>(state->calc);
  return calc->calculate(expression).number().floatValue();
}

const char *calculate_string(const char *expression, QalculateState *state) {
  if (!expression || !state || !state->calc) {
    return nullptr;
  }

  try {
    Calculator *calc = reinterpret_cast<Calculator *>(state->calc);
    EvaluationOptions eo;
    eo.parse_options.angle_unit = ANGLE_UNIT_RADIANS;

    MathStructure result = calc->calculate(expression, eo);

    PrintOptions po;
    po.number_fraction_format = FRACTION_DECIMAL;

    std::string result_str = result.print(po);

    // Выделяем память для строки результата
    char *c_str = static_cast<char *>(malloc(result_str.length() + 1));
    if (c_str) {
      strcpy(c_str, result_str.c_str());
    }
    return c_str;
  } catch (...) {
    return nullptr;
  }
}

void destroy(QalculateState *state) {
  if (state && state->calc) {
    Calculator *calc = reinterpret_cast<Calculator *>(state->calc);
    delete calc;
    state->calc = nullptr;
  }
}

void load_global_definitions(QalculateState *state){
    reinterpret_cast<Calculator*>(state)->loadGlobalDefinitions();
}

}
