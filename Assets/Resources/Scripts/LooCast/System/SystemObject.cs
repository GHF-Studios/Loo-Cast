

namespace LooCast.System
{
    using LooCast.System.Identification;

    public class SystemObject
    {
        #region Properties
        public SystemObjectIdentifier Identifier => identifier;
        #endregion

        #region Fields
        private SystemObjectIdentifier identifier;
        private object instance;
        #endregion
    }
}
