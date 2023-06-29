using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System
{
    public class ManagerMonoBehaviour : ExtendedMonoBehaviour
    {
        #region Static Methods
        public static ManagerMonoBehaviour Create(string managerName)
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
