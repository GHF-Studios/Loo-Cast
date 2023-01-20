using System;
using UnityEngine;

namespace LooCast.Identifier
{
    public interface IIdentifiableNamespace : IIdentifiable
    {
        string Namespace { get; }
    }
}
