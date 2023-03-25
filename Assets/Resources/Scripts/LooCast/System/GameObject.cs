

namespace LooCast.System
{
    using LooCast.System.Identification;

    public class GameObject
    {
        #region Properties
        public GameObjectIdentifier Identifier => identifier;
        #endregion

        #region Fields
        private GameObjectIdentifier identifier;
        private UnityEngine.Object instance;
        #endregion
    }
}
