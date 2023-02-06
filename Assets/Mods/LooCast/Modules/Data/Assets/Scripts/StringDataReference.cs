using System;
using UnityEngine.Events;

namespace LooCast.Data
{
    [Serializable]
    public class StringDataReference
    {
        public bool UseConstant = false;
        public string ConstantValue;
        public DynamicStringData Variable;
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

        public static string[] Evaluate(StringDataReference[] stringDataReferences)
        {
            string[] evaluatedValues = new string[stringDataReferences.Length];
            for (int i = 0; i < stringDataReferences.Length; i++)
            {
                evaluatedValues[i] = stringDataReferences[i].Value;
            }
            return evaluatedValues;
        }
    } 
}
