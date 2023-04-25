using System.Collections.Generic;

namespace LooCast.System
{
    public interface ILooCastObject
    {
        #region Properties
        public IIdentifier Identifier { get; }
        public IType Type { get; }
        public IMetaData MetaData { get; }
        public IData Data { get; }
        #endregion

        #region Methods
        public bool Validate();
        #endregion
    }
}
