using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Data
{
    [CreateAssetMenu(fileName = "FloatData", menuName = "Data/FloatData", order = 0)]
    public class FloatData : ScriptableObject
    {
        [SerializeField] private float value;
        public float Value
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
        public UnityEvent OnValueChanged;
    } 
}
