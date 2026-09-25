// Auto-generated algebraic tests by Trenza DSL Compiler (Strand 2 - TS)
// DO NOT EDIT — regenerate from .trz source
// Run with: npx vitest

import { describe, it, expect, vi } from 'vitest';
import { Contexto, System } from './CLI_Trenza_out';
import type { Effects } from './CLI_Trenza_out';
import { handle_terminal_ejecutar, handle_lector_fs_iniciar_lectura, handle_validador_iniciar_validacion, handle_generador_iniciar_generacion_rust, handle_generador_iniciar_generacion_ts, handle_logger_entrar, handle_resultado_entrar } from './CLI_Trenza_out';

const noOpEffects: Effects = {
    comprobarIntegridad: () => {},
    emitirCodigoRust: () => {},
    emitirCodigoTS: () => {},
    imprimirAyudaCLI: () => {},
    imprimirErrorFatal: () => {},
    imprimirMensajeExito: () => {},
    leerYEvaluarArgumentos: () => {},
    leerYParsear: () => {},
};

function mockEffects(): Effects {
    return {
        comprobarIntegridad: vi.fn(),
        emitirCodigoRust: vi.fn(),
        emitirCodigoTS: vi.fn(),
        imprimirAyudaCLI: vi.fn(),
        imprimirErrorFatal: vi.fn(),
        imprimirMensajeExito: vi.fn(),
        leerYEvaluarArgumentos: vi.fn(),
        leerYParsear: vi.fn(),
    } as unknown as Effects;
}

describe('Transitions', () => {
    it('EsperandoComando on comandoGenerate → ParseandoArchivo', () => {
        const sys = new System(Contexto.EsperandoComando, noOpEffects);
        sys.handleEvent('comandoGenerate');
        expect(sys.state).toBe(Contexto.ParseandoArchivo);
    });

    it('EsperandoComando on comandoInvalido → MostrarAyuda', () => {
        const sys = new System(Contexto.EsperandoComando, noOpEffects);
        sys.handleEvent('comandoInvalido');
        expect(sys.state).toBe(Contexto.MostrarAyuda);
    });

    it('ParseandoArchivo on parseoExitoso → VerificandoReglas', () => {
        const sys = new System(Contexto.ParseandoArchivo, noOpEffects);
        sys.handleEvent('parseoExitoso');
        expect(sys.state).toBe(Contexto.VerificandoReglas);
    });

    it('ParseandoArchivo on errorSintaxis → ErrorFatal', () => {
        const sys = new System(Contexto.ParseandoArchivo, noOpEffects);
        sys.handleEvent('errorSintaxis');
        expect(sys.state).toBe(Contexto.ErrorFatal);
    });

    it('VerificandoReglas on validacionExitosa → GenerandoStrands', () => {
        const sys = new System(Contexto.VerificandoReglas, noOpEffects);
        sys.handleEvent('validacionExitosa');
        expect(sys.state).toBe(Contexto.GenerandoStrands);
    });

    it('VerificandoReglas on validacionFallida → ErrorFatal', () => {
        const sys = new System(Contexto.VerificandoReglas, noOpEffects);
        sys.handleEvent('validacionFallida');
        expect(sys.state).toBe(Contexto.ErrorFatal);
    });

    it('GenerandoStrands on generacionCompleta → Exito', () => {
        const sys = new System(Contexto.GenerandoStrands, noOpEffects);
        sys.handleEvent('generacionCompleta');
        expect(sys.state).toBe(Contexto.Exito);
    });

    it('ErrorFatal on finalizar → EsperandoComando', () => {
        const sys = new System(Contexto.ErrorFatal, noOpEffects);
        sys.handleEvent('finalizar');
        expect(sys.state).toBe(Contexto.EsperandoComando);
    });

    it('Exito on finalizar → EsperandoComando', () => {
        const sys = new System(Contexto.Exito, noOpEffects);
        sys.handleEvent('finalizar');
        expect(sys.state).toBe(Contexto.EsperandoComando);
    });

    it('MostrarAyuda on finalizar → EsperandoComando', () => {
        const sys = new System(Contexto.MostrarAyuda, noOpEffects);
        sys.handleEvent('finalizar');
        expect(sys.state).toBe(Contexto.EsperandoComando);
    });

});

describe('Overlay Stack', () => {
});

describe('Handlers', () => {
    it('EsperandoComando terminal ejecutar invokes leerYEvaluarArgumentos and returns event', () => {
        const effects = mockEffects();
        const event = handle_terminal_ejecutar(Contexto.EsperandoComando, {} as ArgumentosUsuario, effects);
        expect(event).toBe('leerYEvaluarArgumentos');
        expect((effects as any).leerYEvaluarArgumentos).toHaveBeenCalled();
    });

    it('EsperandoComando lector_fs iniciar_lectura throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_lector_fs_iniciar_lectura(Contexto.EsperandoComando, {} as ArgumentosUsuario, effects)).toThrow('Forbidden');
    });

    it('EsperandoComando validador iniciar_validacion throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_validador_iniciar_validacion(Contexto.EsperandoComando, {} as AST, effects)).toThrow('Forbidden');
    });

    it('EsperandoComando generador iniciar_generacion_rust throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_generador_iniciar_generacion_rust(Contexto.EsperandoComando, {} as AST, effects)).toThrow('Forbidden');
    });

    it('EsperandoComando generador iniciar_generacion_ts throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_generador_iniciar_generacion_ts(Contexto.EsperandoComando, {} as AST, effects)).toThrow('Forbidden');
    });

    it('EsperandoComando logger entrar throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_logger_entrar(Contexto.EsperandoComando, {} as ErrorCompilacion, effects)).toThrow('Forbidden');
    });

    it('EsperandoComando resultado entrar throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_resultado_entrar(Contexto.EsperandoComando, {} as ArgumentosUsuario, effects)).toThrow('Forbidden');
    });

    it('ParseandoArchivo lector_fs iniciar_lectura invokes leerYParsear and returns event', () => {
        const effects = mockEffects();
        const event = handle_lector_fs_iniciar_lectura(Contexto.ParseandoArchivo, {} as ArgumentosUsuario, effects);
        expect(event).toBe('leerYParsear');
        expect((effects as any).leerYParsear).toHaveBeenCalled();
    });

    it('ParseandoArchivo terminal ejecutar throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_terminal_ejecutar(Contexto.ParseandoArchivo, {} as ArgumentosUsuario, effects)).toThrow('Forbidden');
    });

    it('ParseandoArchivo validador iniciar_validacion throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_validador_iniciar_validacion(Contexto.ParseandoArchivo, {} as AST, effects)).toThrow('Forbidden');
    });

    it('ParseandoArchivo generador iniciar_generacion_rust throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_generador_iniciar_generacion_rust(Contexto.ParseandoArchivo, {} as AST, effects)).toThrow('Forbidden');
    });

    it('ParseandoArchivo generador iniciar_generacion_ts throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_generador_iniciar_generacion_ts(Contexto.ParseandoArchivo, {} as AST, effects)).toThrow('Forbidden');
    });

    it('ParseandoArchivo logger entrar throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_logger_entrar(Contexto.ParseandoArchivo, {} as ErrorCompilacion, effects)).toThrow('Forbidden');
    });

    it('ParseandoArchivo resultado entrar throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_resultado_entrar(Contexto.ParseandoArchivo, {} as ArgumentosUsuario, effects)).toThrow('Forbidden');
    });

    it('VerificandoReglas validador iniciar_validacion invokes comprobarIntegridad and returns event', () => {
        const effects = mockEffects();
        const event = handle_validador_iniciar_validacion(Contexto.VerificandoReglas, {} as AST, effects);
        expect(event).toBe('comprobarIntegridad');
        expect((effects as any).comprobarIntegridad).toHaveBeenCalled();
    });

    it('VerificandoReglas terminal ejecutar throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_terminal_ejecutar(Contexto.VerificandoReglas, {} as ArgumentosUsuario, effects)).toThrow('Forbidden');
    });

    it('VerificandoReglas lector_fs iniciar_lectura throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_lector_fs_iniciar_lectura(Contexto.VerificandoReglas, {} as ArgumentosUsuario, effects)).toThrow('Forbidden');
    });

    it('VerificandoReglas generador iniciar_generacion_rust throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_generador_iniciar_generacion_rust(Contexto.VerificandoReglas, {} as AST, effects)).toThrow('Forbidden');
    });

    it('VerificandoReglas generador iniciar_generacion_ts throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_generador_iniciar_generacion_ts(Contexto.VerificandoReglas, {} as AST, effects)).toThrow('Forbidden');
    });

    it('VerificandoReglas logger entrar throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_logger_entrar(Contexto.VerificandoReglas, {} as ErrorCompilacion, effects)).toThrow('Forbidden');
    });

    it('VerificandoReglas resultado entrar throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_resultado_entrar(Contexto.VerificandoReglas, {} as ArgumentosUsuario, effects)).toThrow('Forbidden');
    });

    it('GenerandoStrands generador iniciar_generacion_rust invokes emitirCodigoRust and returns event', () => {
        const effects = mockEffects();
        const event = handle_generador_iniciar_generacion_rust(Contexto.GenerandoStrands, {} as AST, effects);
        expect(event).toBe('emitirCodigoRust');
        expect((effects as any).emitirCodigoRust).toHaveBeenCalled();
    });

    it('GenerandoStrands generador iniciar_generacion_ts invokes emitirCodigoTS and returns event', () => {
        const effects = mockEffects();
        const event = handle_generador_iniciar_generacion_ts(Contexto.GenerandoStrands, {} as AST, effects);
        expect(event).toBe('emitirCodigoTS');
        expect((effects as any).emitirCodigoTS).toHaveBeenCalled();
    });

    it('GenerandoStrands terminal ejecutar throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_terminal_ejecutar(Contexto.GenerandoStrands, {} as ArgumentosUsuario, effects)).toThrow('Forbidden');
    });

    it('GenerandoStrands lector_fs iniciar_lectura throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_lector_fs_iniciar_lectura(Contexto.GenerandoStrands, {} as ArgumentosUsuario, effects)).toThrow('Forbidden');
    });

    it('GenerandoStrands validador iniciar_validacion throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_validador_iniciar_validacion(Contexto.GenerandoStrands, {} as AST, effects)).toThrow('Forbidden');
    });

    it('GenerandoStrands logger entrar throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_logger_entrar(Contexto.GenerandoStrands, {} as ErrorCompilacion, effects)).toThrow('Forbidden');
    });

    it('GenerandoStrands resultado entrar throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_resultado_entrar(Contexto.GenerandoStrands, {} as ArgumentosUsuario, effects)).toThrow('Forbidden');
    });

    it('ErrorFatal logger entrar invokes imprimirErrorFatal and returns event', () => {
        const effects = mockEffects();
        const event = handle_logger_entrar(Contexto.ErrorFatal, {} as ErrorCompilacion, effects);
        expect(event).toBe('imprimirErrorFatal');
        expect((effects as any).imprimirErrorFatal).toHaveBeenCalled();
    });

    it('ErrorFatal terminal ejecutar throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_terminal_ejecutar(Contexto.ErrorFatal, {} as ArgumentosUsuario, effects)).toThrow('Forbidden');
    });

    it('ErrorFatal lector_fs iniciar_lectura throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_lector_fs_iniciar_lectura(Contexto.ErrorFatal, {} as ArgumentosUsuario, effects)).toThrow('Forbidden');
    });

    it('ErrorFatal validador iniciar_validacion throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_validador_iniciar_validacion(Contexto.ErrorFatal, {} as AST, effects)).toThrow('Forbidden');
    });

    it('ErrorFatal generador iniciar_generacion_rust throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_generador_iniciar_generacion_rust(Contexto.ErrorFatal, {} as AST, effects)).toThrow('Forbidden');
    });

    it('ErrorFatal generador iniciar_generacion_ts throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_generador_iniciar_generacion_ts(Contexto.ErrorFatal, {} as AST, effects)).toThrow('Forbidden');
    });

    it('ErrorFatal resultado entrar throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_resultado_entrar(Contexto.ErrorFatal, {} as ArgumentosUsuario, effects)).toThrow('Forbidden');
    });

    it('Exito resultado entrar invokes imprimirMensajeExito and returns event', () => {
        const effects = mockEffects();
        const event = handle_resultado_entrar(Contexto.Exito, {} as ArgumentosUsuario, effects);
        expect(event).toBe('imprimirMensajeExito');
        expect((effects as any).imprimirMensajeExito).toHaveBeenCalled();
    });

    it('Exito logger entrar throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_logger_entrar(Contexto.Exito, {} as ErrorCompilacion, effects)).toThrow('Forbidden');
    });

    it('Exito terminal ejecutar throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_terminal_ejecutar(Contexto.Exito, {} as ArgumentosUsuario, effects)).toThrow('Forbidden');
    });

    it('Exito lector_fs iniciar_lectura throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_lector_fs_iniciar_lectura(Contexto.Exito, {} as ArgumentosUsuario, effects)).toThrow('Forbidden');
    });

    it('Exito validador iniciar_validacion throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_validador_iniciar_validacion(Contexto.Exito, {} as AST, effects)).toThrow('Forbidden');
    });

    it('Exito generador iniciar_generacion_rust throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_generador_iniciar_generacion_rust(Contexto.Exito, {} as AST, effects)).toThrow('Forbidden');
    });

    it('Exito generador iniciar_generacion_ts throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_generador_iniciar_generacion_ts(Contexto.Exito, {} as AST, effects)).toThrow('Forbidden');
    });

    it('MostrarAyuda logger entrar invokes imprimirAyudaCLI and returns event', () => {
        const effects = mockEffects();
        const event = handle_logger_entrar(Contexto.MostrarAyuda, {} as ErrorCompilacion, effects);
        expect(event).toBe('imprimirAyudaCLI');
        expect((effects as any).imprimirAyudaCLI).toHaveBeenCalled();
    });

    it('MostrarAyuda terminal ejecutar throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_terminal_ejecutar(Contexto.MostrarAyuda, {} as ArgumentosUsuario, effects)).toThrow('Forbidden');
    });

    it('MostrarAyuda lector_fs iniciar_lectura throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_lector_fs_iniciar_lectura(Contexto.MostrarAyuda, {} as ArgumentosUsuario, effects)).toThrow('Forbidden');
    });

    it('MostrarAyuda validador iniciar_validacion throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_validador_iniciar_validacion(Contexto.MostrarAyuda, {} as AST, effects)).toThrow('Forbidden');
    });

    it('MostrarAyuda generador iniciar_generacion_rust throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_generador_iniciar_generacion_rust(Contexto.MostrarAyuda, {} as AST, effects)).toThrow('Forbidden');
    });

    it('MostrarAyuda generador iniciar_generacion_ts throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_generador_iniciar_generacion_ts(Contexto.MostrarAyuda, {} as AST, effects)).toThrow('Forbidden');
    });

    it('MostrarAyuda resultado entrar throws Forbidden', () => {
        const effects = mockEffects();
        expect(() => handle_resultado_entrar(Contexto.MostrarAyuda, {} as ArgumentosUsuario, effects)).toThrow('Forbidden');
    });

});

describe('Exhaustiveness', () => {
    it('Contexto enum has all contexts', () => {
        expect(Object.keys(Contexto).length).toBe(7);
    });
});
