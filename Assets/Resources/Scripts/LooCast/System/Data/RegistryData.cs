using System;
using System.Collections.Generic;

namespace LooCast.System.Data
{ 
    public class RegistryData<KeyType, ValueType> : IRegistryData
        where KeyType : IIdentifier
        where ValueType : IInstance
    {
        #region Properties
        public IData DataParent
        {
            get
            {
                return registryDataParent;
            }

            set
            {
                registryDataParent = (IRegistryData)value;
            }
        }
        public IRegistryData RegistryDataParent
        {
            get
            {
                return registryDataParent;
            }

            set
            {
                registryDataParent = value;
            }
        }

        public IEnumerable<IData> DataChildren
        {
            get
            {
                return registryDataChildren;
            }

            set
            {
                registryDataChildren = (IEnumerable<IRegistryData>)value;
            }
        }
        public IEnumerable<IRegistryData> RegistryDataChildren
        {
            get
            {
                return registryDataChildren;
            }

            set
            {
                registryDataChildren = value;
            }
        }
        #endregion

        #region Fields
        #endregion

        #region Constructors

        #endregion
    }
}
