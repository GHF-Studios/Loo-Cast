using System;
using UnityEngine.Events;

namespace LooCast.Data
{
    [Serializable]
    public class StringDataReference
    {
        public bool UseConstant = true;
        public string ConstantValue;
        public StringData Variable;
        public UnityEvent OnValueChanged = new UnityEvent();

        public string Value
        {
            get
            {
                return UseConstant ? ConstantValue : Variable.Value;
            }

            set
            {
                if (UseConstant)
                {
                    ConstantValue = value;
                }
                else
                {
                    Variable.Value = value;
                }
                OnValueChanged.Invoke();
            }
        }
    } 
}
