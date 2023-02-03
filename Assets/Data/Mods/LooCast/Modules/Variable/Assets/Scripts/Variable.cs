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
                if (IsInitialized)
                {
                    return value;
                }
                return default(T);
            }

            set
            {
                this.value = value;
                if (IsInitialized)
                {
                    OnValueChanged.Invoke();
                }
            }
        }
        [HideInInspector, SerializeField] private T value;
        public UnityEvent OnValueChanged { get; private set; }
        public readonly bool IsInitialized = false;

        public Variable(T value)
        {
            OnValueChanged = new UnityEvent();
            Value = value;
            IsInitialized = true;
        }
    }
}