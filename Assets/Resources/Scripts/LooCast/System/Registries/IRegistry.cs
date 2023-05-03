using System.Collections.Generic;

namespace LooCast.System.Registries
{
    public interface IRegistry : ILooCastObject
    {
        #region Properties
        public IRegistryData RegistryData { get; set; }
        
        public IRegistry RegistryParent { get; }
        public IEnumerable<IRegistry> RegistryChildren { get; }
        #endregion

        #region Methods
        public void Add(IIdentifier key, IInstance value);
        public bool Remove(IIdentifier key);
        public void Clear();
        #endregion
    }
}
