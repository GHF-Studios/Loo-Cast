using System;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Data
{
    [CreateAssetMenu(fileName = "DynamicIntData", menuName = "Data/DynamicIntData", order = 0)]
    public class DynamicIntData : DynamicData
    {
        #region Properties
        public int Value
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
        public override Type DataType
        {
            get
            {
                return typeof(DynamicIntData);
            }
        }
        public override DataFolder ParentFolder
        {
            get
            {
                return (DataFolder)DataManager.Instance.GetDataFolder(parentFolderID);
            }
        }
        #endregion

        #region Fields
        public UnityEvent OnValueChanged;
        
        [SerializeField] private int value;
        [SerializeField] private string parentFolderID;
        #endregion
    } 
}
