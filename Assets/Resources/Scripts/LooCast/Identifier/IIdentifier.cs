using LooCast.Core;
using System;
using System.Reflection;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Identifier
{
    public interface IIdentifier
    {
        #region Properties
        string ID { get; }
        #endregion
    }
}