using System;

namespace LooCast.System.Identification
{
    public interface ICSharpInstanceIdentifiable : IInstanceIdentifiable
    {
        #region Properties
        ICSharpInstanceIdentifier CSharpInstanceIdentifier { get; }
        #endregion
    }
}
