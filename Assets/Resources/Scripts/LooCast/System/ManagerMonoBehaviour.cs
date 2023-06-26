using System;
using UnityEngine;

namespace LooCast.System
{
    public sealed class ManagerMonoBehaviour : MonoBehaviour
    {
        #region Static Methods
        public static ManagerMonoBehaviour CreateManagerObject(string managerName)
        {
            ManagerMonoBehaviour managerMonoBehaviour = new GameObject($"[{managerName}]").AddComponent<ManagerMonoBehaviour>();
            managerMonoBehaviour.gameObject.layer = 31;
            managerMonoBehaviour.gameObject.tag = "INTERNAL";
            DontDestroyOnLoad(managerMonoBehaviour);
            return managerMonoBehaviour;
        }
        #endregion
    }
}
