using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.MetaData;
    using LooCast.System.Data;
    
    public interface ILooCastObject
    {
        #region Properties
        public IMetaData MetaData { get; set; }
        public IData Data { get; set; }
        #endregion

        #region Methods
        public bool Validate();

        public void PreConstruct();
        public void Construct();
        public void PostConstruct();

        public void PreDestruct();
        public void Destruct();
        public void PostDestruct();
        #endregion
    }
}
