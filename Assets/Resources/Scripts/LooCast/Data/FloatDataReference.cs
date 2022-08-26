using System;
using UnityEngine.Events;

namespace LooCast.Data
{
    [Serializable]
    public class FloatDataReference
    {
        public bool UseConstant = false;
        public float ConstantValue;
        public FloatData Variable;
        public UnityEvent OnValueChanged = new UnityEvent();

        public float Value
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

        public static float[] Evaluate(FloatDataReference[] floatDataReferences)
        {
            float[] evaluatedValues = new float[floatDataReferences.Length];
            for (int i = 0; i < floatDataReferences.Length; i++)
            {
                evaluatedValues[i] = floatDataReferences[i].Value;
            }
            return evaluatedValues;
        }
    } 
}
