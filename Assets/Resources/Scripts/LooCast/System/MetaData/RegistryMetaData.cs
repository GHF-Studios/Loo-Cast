using System;
using System.Collections.Generic;

namespace LooCast.System.MetaData
{
    public class RegistryMetaData<KeyType, ValueType> : IRegistryMetaData
        where KeyType : IIdentifier
        where ValueType : IInstance
    {
    }
}
