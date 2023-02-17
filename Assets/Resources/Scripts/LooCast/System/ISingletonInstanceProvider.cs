namespace LooCast.System
{
    public interface ISingletonInstanceProvider
    {
        #region Properties
        CSharpInstance LooCastInstance { get; }
        #endregion
    }
}