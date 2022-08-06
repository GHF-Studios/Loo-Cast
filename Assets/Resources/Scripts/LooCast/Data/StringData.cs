using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Data
{
    [CreateAssetMenu(fileName = "StringData", menuName = "Data/StringData", order = 0)]
    public class StringData : ScriptableObject
    {
        [SerializeField] private string value;
        public string Value
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
