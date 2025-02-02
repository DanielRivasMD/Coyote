////////////////////////////////////////////////////////////////////////////////////////////////////

/// Daedalus
/// In Greek mythology, Daedalus (UK: /ˈdiːdələs/, US: /ˈdɛdələs/; Greek:
/// Δαίδαλος; Latin: Daedalus; Etruscan: Taitale) was a skillful architect and
/// craftsman, seen as a symbol of wisdom, knowledge and power. He is the father
/// of Icarus, the uncle of Perdix, and possibly also the father of Iapyx.
/// Parse interactive methods
#[macro_export]
macro_rules! daedalus {
  // declare implementation for type
  // interactive methods for referenced fields
  // implement 'get_owned', 'get_ref' & 'update'
    (
      $vis: vis,
      $stt: ident;
      $(
        $fld: tt;
        $fown: ident, $fref: ident, $fnup: ident - $town: ty, $tref: ty
      )*
    ) => {
      impl $stt {
        $(
          daedalus!(getown |> $vis, $fld; $fown - $town);
          daedalus!(getref |> $vis, $fld; $fref - $tref);
          daedalus!(update |> $vis, $fld; $fnup - $town);
        )*
      }
    };

  // declare implementation for type
  // interactive methods for non-referenced fields
  // implement 'get' & 'update'
    (
      $vis: vis,
      $stt: ident;
      $(
        $fld: tt;
        $fint: ident, $fnup: ident - $ty: ty
      )*
    ) => {
      impl $stt {
        $(
          daedalus!(getint |> $vis, $fld; $fint - $ty);
          daedalus!(update |> $vis, $fld; $fnup - $ty);
        )*
      }
    };

  // declare implementation for type
  // interactive methods for referenced fields
  // implement 'get_own'
    (
      getown |> $vis: vis, $fld: tt; $fown: ident - $town: ty
    ) => {
      $vis fn $fown(&self) -> anyResult<$town> {
        Ok(self.$fld.to_owned())
      }
    };

  // declare implementation for type
  // interactive methods for referenced fields
  // implement 'get_ref'
    (
      getref |> $vis: vis, $fld: tt; $fref: ident - $tref: ty
    ) => {
      $vis fn $fref(&self) -> anyResult<$tref> {
        Ok(&self.$fld)
      }
    };

  // declare implementation for type
  // interactive methods for non-referenced fields
  // implement 'get'
    (
      getint |> $vis: vis, $fld: tt; $fint: ident - $tint: ty
    ) => {
      $vis fn $fint(&self) -> anyResult<$tint> {
        Ok(self.$fld)
      }
    };

  // declare implementation for type
  // interactive methods
  // implement 'update'
    (
      update |> $vis: vis, $fld: tt; $fnup: ident - $tyup: ty
    ) => {
      $vis fn $fnup(&mut self, up: $tyup) -> anyResult<()> {
        self.$fld = up;
        Ok(())
      }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
