using System.Collections.Generic;

namespace LooCast.System.Registries
{
    using LooCast.System.MetaData;
    using LooCast.System.Data;
    
    public interface IRegistry : ILooCastObject
    {
        #region Properties
        public IRegistryMetaData RegistryMetaData { get; set; }
        public IRegistryData RegistryData { get; set; }
        #endregion

        #region Methods
        public void Add(IIdentifier key, IInstance value);
        public bool Remove(IIdentifier key);
        public void Clear();
        #endregion
    }
}
