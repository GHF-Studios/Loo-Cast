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
            Value = value;
        }
    }
}