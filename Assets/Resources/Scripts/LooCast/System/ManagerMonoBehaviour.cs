using System;
using UnityEngine;

namespace LooCast.System
{
    public sealed class ManagerMonoBehaviour : MonoBehaviour
    {
        #region Static Methods
        public static ManagerMonoBehaviour CreateManagerObject(string name, int layer, string tag)
        {
            ManagerMonoBehaviour managerMonoBehaviour = new GameObject(name).AddComponent<ManagerMonoBehaviour>();
            managerMonoBehaviour.gameObject.layer = layer;
            managerMonoBehaviour.gameObject.tag = tag;
            DontDestroyOnLoad(managerMonoBehaviour);
            return managerMonoBehaviour;
        }
        #endregion
    }
}
