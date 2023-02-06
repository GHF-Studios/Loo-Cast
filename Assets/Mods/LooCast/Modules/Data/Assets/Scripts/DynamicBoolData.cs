using System;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Data
{
    [CreateAssetMenu(fileName = "DynamicBoolData", menuName = "Data/DynamicBoolData", order = 0)]
    public class DynamicBoolData : DynamicData
    {
        #region Properties
        public bool Value
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
                return typeof(DynamicBoolData);
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
        
        [SerializeField] private bool value;
        [SerializeField] private string parentFolderID;
        #endregion
    } 
}
