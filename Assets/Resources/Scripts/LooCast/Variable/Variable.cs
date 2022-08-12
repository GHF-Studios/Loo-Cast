using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Variable
{
    public abstract class Variable<T>
    {
        public T Value
        {
            get
            {
                return value;
            }

            set
            {
                this.value = value;
                OnValueChanged.Invoke();
            }
        }
        private T value;
        public UnityEvent OnValueChanged;

        public Variable(T value)
        {
            OnValueChanged = new UnityEvent();
            Value = value;
        }

        public static T[] Evaluate(Variable<T>[] valueVariables)
        {
            T[] evaluatedValues = new T[valueVariables.Length];
            for (int i = 0; i < valueVariables.Length; i++)
            {
                evaluatedValues[i] = valueVariables[i].Value;
            }
            return evaluatedValues;
        }
    }
}