using System;
using UnityEngine;

namespace LooCast.System
{
    public sealed class ManagerObject : MonoBehaviour
    {
        #region Static Methods
        public static ManagerObject CreateManagerObject(string name, int layer, string tag)
        {
            ManagerObject managerObject = new GameObject(name).AddComponent<ManagerObject>();
            managerObject.gameObject.layer = layer;
            managerObject.gameObject.tag = tag;
            DontDestroyOnLoad(managerObject);
            return managerObject;
        }
        #endregion
    }
}
