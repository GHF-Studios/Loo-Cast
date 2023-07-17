using UnityEngine;

namespace LooCast.Core
{
    public sealed class UnityBridge
    {
        #region Properties
        public GameObject RootGameObject { get; private set; }
        #endregion

        #region Constructors
        public UnityBridge()
        {
            RootGameObject = new GameObject();
        }
        #endregion

        #region Methods
        public void Terminate()
        {
            if (RootGameObject is not null)
            {
                GameObject.Destroy(RootGameObject);
                RootGameObject = null;
            }
        }
        #endregion
    }
}
