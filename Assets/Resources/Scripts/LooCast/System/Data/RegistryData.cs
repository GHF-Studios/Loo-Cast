using System;
using System.Collections.Generic;

namespace LooCast.System.Data
{
    public class RegistryData<KeyType, ValueType> : IRegistryData
        where KeyType : IIdentifier
        where ValueType : IInstance
    {
    }
}
