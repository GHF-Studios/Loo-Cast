using System.Collections.Generic;

namespace LooCast.System
{
    public interface ILooCastObject
    {
        #region Properties
        public IMetaData MetaData { get; set; }
        public IData Data { get; set; }
        #endregion

        #region Methods
        public bool Validate();
        #endregion
    }
}
