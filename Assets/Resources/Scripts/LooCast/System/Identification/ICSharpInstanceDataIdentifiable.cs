using System;

namespace LooCast.System.Identification
{
    public interface ICSharpInstanceDataIdentifiable : IInstanceDataIdentifiable
    {
        #region Properties
        ICSharpInstanceDataIdentifier CSharpInstanceDataIdentifier { get; }
        #endregion
    }
}
